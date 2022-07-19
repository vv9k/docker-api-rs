package docker_api.codegen;

import java.lang.Class;
import java.lang.reflect.Field;
import io.swagger.codegen.v3.*;
import io.swagger.codegen.v3.generators.DefaultCodegenConfig;
import io.swagger.codegen.v3.generators.util.OpenAPIUtil;
import io.swagger.v3.oas.models.Operation;
import io.swagger.v3.oas.models.media.*;
import org.apache.commons.lang3.StringUtils;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;

public class DockerApiCodegen extends DefaultCodegenConfig {

  static Logger LOGGER = LoggerFactory.getLogger(DockerApiCodegen.class);

  static Map<String, String> SPECIAL_TYPES = new HashMap<String, String>() {
    {
    }
  };

  public CodegenType getTag() {
    return CodegenType.OTHER;
  }

  public String getName() {
    return "docker-api-codegen";
  }

  public String getHelp() {
    return "Generates Docker API stubs";
  }

  public DockerApiCodegen() {
    super();

    additionalProperties.put(CodegenConstants.PACKAGE_VERSION, "0.1.0");
    additionalProperties.put(CodegenConstants.PACKAGE_NAME, "docker-api-stubs");

    /**
     * Template Location. This is the location which templates will be read from.
     * The generator
     * will use the resource stream to attempt to read the templates.
     */
    templateDir = "rust";

    /**
     * Reserved words. Override this with reserved words specific to your language
     */
    setReservedWordsLowerCase(
        Arrays.asList(
            "abstract", "alignof", "as", "become", "box",
            "break", "const", "continue", "crate", "do",
            "else", "enum", "extern", "false", "final",
            "fn", "for", "if", "impl", "in",
            "let", "loop", "macro", "match", "mod",
            "move", "mut", "offsetof", "override", "priv",
            "proc", "pub", "pure", "ref", "return",
            "Self", "self", "sizeof", "static", "struct",
            "super", "trait", "true", "type", "typeof",
            "unsafe", "unsized", "use", "virtual", "where",
            "while", "yield"));

    /**
     * Supporting Files. You can write single files for the generator with the
     * entire object tree available. If the input file has a suffix of `.mustache
     * it will be processed by the template engine. Otherwise, it will be copied
     */
    supportingFiles.add(new SupportingFile("lib.mustache", "src", "lib.rs"));
    supportingFiles.add(new SupportingFile("models.mustache", "src", "models.rs"));
    supportingFiles.add(new SupportingFile("Cargo.mustache", "", "Cargo.toml"));
    supportingFiles.add(new SupportingFile("gitignore.mustache", "", ".gitignore"));

    defaultIncludes = new HashSet<String>(
        Arrays.asList(
            "map",
            "array"));

    /**
     * Language Specific Primitives. These types will not trigger imports by
     * the client generator
     */
    languageSpecificPrimitives = new HashSet<String>(
        Arrays.asList(
            "i8", "i16", "i32", "i64",
            "u8", "u16", "u32", "u64",
            "f32", "f64", "str", "String",
            "char", "bool", "Vec<u8>", "File", "BigDecimal"));

    instantiationTypes.clear();

    typeMapping.clear();
    typeMapping.put("integer", "i32");
    typeMapping.put("long", "i64");
    typeMapping.put("number", "f32");
    typeMapping.put("float", "f32");
    typeMapping.put("double", "f64");
    typeMapping.put("boolean", "bool");
    typeMapping.put("string", "String");
    typeMapping.put("UUID", "String");
    typeMapping.put("date", "string");
    typeMapping.put("DateTime", "String");
    typeMapping.put("password", "String");
    typeMapping.put("file", "File");
    typeMapping.put("binary", "Vec<u8>");
    typeMapping.put("ByteArray", "String");
    typeMapping.put("object", "Value");
  }

  /**
   * Escapes a reserved word as defined in the `reservedWords` array. Handle
   * escaping
   * those terms here. This logic is only called if a variable matches the
   * reserved words
   * 
   * @return the escaped term
   */
  @Override
  public String escapeReservedWord(String name) {
    return "_" + name; // add an underscore to the name
  }

  public Map<String, String> createMapping(String key, String value) {
    Map<String, String> customImport = new HashMap<>();
    customImport.put(key, value);
    return customImport;
  }

  public void addOperationToGroup(String tag, String resourcePath, Operation operation, CodegenOperation co,
      Map<String, List<CodegenOperation>> operations) {
    if (co.path.startsWith("/")) {
      co.path = co.path.substring(1);
    }

    super.addOperationToGroup(tag, co.path, operation, co, operations);
  }

  @Override
  public Map<String, Object> postProcessAllModels(Map<String, Object> objs) {

    return super.postProcessAllModels(objs);
  }

  private String formatParamExample(String example) {
    StringBuilder ex = new StringBuilder("/// Example:\n");
    if (example.contains("\n")) {
      for (String part : example.split("\n")) {
        ex.append("/// ");
        ex.append(part);
        ex.append('\n');
      }
      return ex.toString();
    } else {
      return "/// " + example;
    }
  }

  private CodegenModel mapClassName(CodegenModel model) {
    if (model.classname != null) {
      LOGGER.error("Mapping " + model.classname);
      if (SPECIAL_TYPES.containsKey(model.classname)) {
        LOGGER.error("Mapping to " + SPECIAL_TYPES.get(model.classname));
        model.classname = SPECIAL_TYPES.get(model.classname);
      }
    }

    return model;
  }

  @SuppressWarnings("unchecked")
  @Override
  public Map<String, Object> postProcessModels(Map<String, Object> objs) {
    List<Map<String, Object>> models = (List<Map<String, Object>>) objs.get("models");

    for (Map<String, Object> m : models) {
      Object v = m.get("model");
      if (v instanceof CodegenModel) {
        CodegenModel model = (CodegenModel) v;

         printAllFields(model);
        model = mapClassName(model);

        if (model.parent != null) {
          if (model.parent.startsWith("null")) {
            model.parent = model.parent.substring(4);
          }
        }

        for (CodegenProperty param : model.vars) {

            if (model.name == "SystemInfo")  {
            LOGGER.error("Printing all fields of " + model.name);
                printAllFields(param);
            }
          if (param.example != null) {
            String ret = formatParamExample(param.example);
            if (ret != null)
              param.example = ret;
          }
        }
      } else {
        LOGGER.error("Not a model: ", m.getClass().getName());
      }
    }

    return postProcessModelsEnum(objs);
  }

  @Override
  public String toVarName(String name) {
    // replace - with _ e.g. created-at => created_at
    name = sanitizeName(name.replaceAll("-", "_"));

    // if it's all uppper case, do nothing
    if (name.matches("^[A-Z_]*$"))
      return name;

    // snake_case, e.g. PetId => pet_id
    name = underscore(name);

    // for reserved word or word starting with number, append _
    if (isReservedWord(name))
      name = escapeReservedWord(name);

    // for reserved word or word starting with number, append _
    if (name.matches("^\\d.*"))
      name = "var_" + name;

    return name;
  }

  @Override
  public String toParamName(String name) {
    return toVarName(name);
  }

  @Override
  public String toModelName(String name) {
    // camelize the model name
    // phone_number => PhoneNumber
    return camelize(toModelFilename(name));
  }

  @Override
  public String toModelFilename(String name) {
    if (!StringUtils.isEmpty(modelNamePrefix)) {
      name = modelNamePrefix + "_" + name;
    }

    if (!StringUtils.isEmpty(modelNameSuffix)) {
      name = name + "_" + modelNameSuffix;
    }

    name = sanitizeName(name);

    // model name cannot use reserved keyword, e.g. return
    if (isReservedWord(name)) {
      LOGGER.warn("Reserved word `" + name + "` cannot be used as model name. Renamed to " + ("model_" + name));
      name = "model_" + name;
    }

    // model name starts with number
    if (name.matches("^\\d.*")) {
      LOGGER.warn(
          "Model name `" + name + "` starts with number cannot be used as model name. Renamed to " + ("model_" + name));
      name = "model_" + name; // e.g. 200Response => Model200Response (after camelize)
    }

    return underscore(name);
  }

  @SuppressWarnings("rawtypes")
  public String getSchemaType(Schema schema) {
    String schemaType = super.getSchemaType(schema);
    if (schema.get$ref() != null) {
      Schema refSchema = OpenAPIUtil.getSchemaFromName(schemaType, this.openAPI);
      if (refSchema != null && !this.isObjectSchema(refSchema)) {
        schemaType = super.getSchemaType(refSchema);
      }
    }

    String type = schemaType;
    if (this.typeMapping.containsKey(schemaType)) {
      type = (String) this.typeMapping.get(schemaType);
      if (this.languageSpecificPrimitives.contains(type)) {
        return type;
      }
    }

    return type;
  }

  @SuppressWarnings("rawtypes")
  @Override
  public String getTypeDeclaration(Schema schema) {
    Schema inner;
    String schemaType = this.getSchemaType(schema);

    if (schema instanceof ArraySchema) {
      ArraySchema arraySchema = (ArraySchema) schema;
      inner = arraySchema.getItems();
      return "Vec<" + this.getTypeDeclaration(inner) + ">";
    } else if (schema instanceof MapSchema) {
      MapSchema mapSchema = (MapSchema) schema;
      inner = (Schema) mapSchema.getAdditionalProperties();
      return "HashMap<String, " + this.getTypeDeclaration(inner) + ">";
    } else if (schema instanceof StringSchema) {
      return "String";
    } else if (schema instanceof NumberSchema) {
      return "f32";
    } else if (schema instanceof IntegerSchema) {
      return "i64";
    } else if (schema instanceof BooleanSchema) {
      return "bool";
    } else if (schema instanceof DateTimeSchema) {
      return "DateTime<Utc>";
    } else if (schema instanceof DateSchema) {
      schema.setFormat("date");
      schema.setTitle("serde(with=date_serializer)");
      return "Date<Utc>";
    } else {
      if (this.typeMapping.containsKey(schemaType)) {
        return (String) this.typeMapping.get(schemaType);
      } else if (schema.get$ref() != null) {
        String[] refh = schema.get$ref().split("/");
        return String.format("%s", this.toModelName(refh[refh.length - 1]));
      } else if (this.typeMapping.containsValue(schemaType)) {
        return schemaType;
      } else {
        String modelName = this.toModelName(schemaType);

        LOGGER.warn(
            "could not resolve given type (schema type: " + schemaType + ", model name: " + modelName
                + "). The generated code is probably faulty. Check the schema!");

        return this.languageSpecificPrimitives.contains(schemaType) ? "" + schemaType
            : "" + modelName;
      }
    }
  }

  @Override
  public String toOperationId(String operationId) {
    String sanitizedOperationId = sanitizeName(operationId);

    // method name cannot use reserved keyword, e.g. return
    if (isReservedWord(sanitizedOperationId)) {
      sanitizedOperationId = "call_" + sanitizedOperationId;
      LOGGER.debug("Reserved word `" + operationId + "` cannot be used as method name. Renamed to "
          + underscore(sanitizedOperationId));
    }

    return underscore(sanitizedOperationId);
  }

  @Override
  public void postProcessParameter(CodegenParameter parameter) {
    super.postProcessParameter(parameter);
    if (parameter.getDataType().equals("DateTime")) {
      parameter.dataFormat = "datetime";
      if (parameter.example == null)
        parameter.example = "2019-03-19T18:38:33.131642+03:00";
      else {
        String ret = formatParamExample(parameter.example);
        if (ret != null) {
          parameter.example = ret;
        }
      }
    }
  }

  @Override
  protected boolean needToImport(String type) {
    return !defaultIncludes.contains(type)
        && !languageSpecificPrimitives.contains(type);
  }

  @Override
  public String escapeQuotationMark(String input) {
    // remove " to avoid code injection
    return input.replace("\"", "");
  }

  @Override
  public String escapeUnsafeCharacters(String input) {
    return input.replace("*/", "*_/").replace("/*", "/_*");
  }

  @Override
  public String toEnumValue(String value, String datatype) {
    if ("int".equals(datatype) || "double".equals(datatype) || "float".equals(datatype)) {
      return value;
    } else {
      return escapeText(value);
    }
  }

  @Override
  public String toEnumDefaultValue(String value, String datatype) {
    return datatype + "_" + value;
  }

  @Override
  public String toEnumVarName(String name, String datatype) {
    if (name.length() == 0) {
      return "EMPTY";
    }

    // number
    if ("int".equals(datatype) || "double".equals(datatype) || "float".equals(datatype)) {
      String varName = name;
      varName = varName.replaceAll("-", "MINUS_");
      varName = varName.replaceAll("\\+", "PLUS_");
      varName = varName.replaceAll("\\.", "_DOT_");
      return varName;
    }

    // for symbol, e.g. $, #
    if (getSymbolName(name) != null) {
      return getSymbolName(name).toUpperCase();
    }

    // string
    String enumName = sanitizeName(underscore(name).toUpperCase());
    enumName = enumName.replaceFirst("^_", "");
    enumName = enumName.replaceFirst("_$", "");

    if (isReservedWord(enumName) || enumName.matches("\\d.*")) { // reserved word or starts with number
      return escapeReservedWord(enumName);
    } else {
      return enumName;
    }
  }

  @Override
  public String toEnumName(CodegenProperty property) {
    String enumName = underscore(toModelName(property.name)).toUpperCase();

    // remove [] for array or map of enum
    enumName = enumName.replace("[]", "");

    if (enumName.matches("\\d.*")) { // starts with number
      return "_" + enumName;
    } else {
      return enumName;
    }
  }

  @Override
  protected String getTemplateDir() {
    return templateDir;
  }

  @Override
  public String getDefaultTemplateDir() {
    return templateDir;
  }

  // ####################################################################################################
  // Used for debugging
  //

  private static Collection<Field> getAllFields(Class<?> type) {
    TreeSet<Field> fields = new TreeSet<Field>(
        new Comparator<Field>() {
          @Override
          public int compare(Field o1, Field o2) {
            int res = o1.getName().compareTo(o2.getName());
            if (0 != res) {
              return res;
            }
            res = o1.getDeclaringClass().getSimpleName().compareTo(o2.getDeclaringClass().getSimpleName());
            if (0 != res) {
              return res;
            }
            res = o1.getDeclaringClass().getName().compareTo(o2.getDeclaringClass().getName());
            return res;
          }
        });
    for (Class<?> c = type; c != null; c = c.getSuperclass()) {
      fields.addAll(Arrays.asList(c.getDeclaredFields()));
    }
    return fields;
  }

  private static void printAllFields(Object obj) {
    for (Field field : getAllFields(obj.getClass())) {
      field.setAccessible(true);
      String name = field.getName();
      Object value = null;
      try {
        value = field.get(obj);
      } catch (IllegalArgumentException | IllegalAccessException e) {
        e.printStackTrace();
      }
      System.out.printf("%s %s.%s = %s;\n", value == null ? " " : "*", field.getDeclaringClass().getSimpleName(), name,
          value);
    }
  }

  // ####################################################################################################
}
